package anoncred.wrapper

import anoncreds_wrapper.CredentialDefinitionConfig
import anoncreds_wrapper.Issuer
import anoncreds_wrapper.PresentationRequest
import anoncreds_wrapper.Prover
import anoncreds_wrapper.Schema
import anoncreds_wrapper.SignatureType
import kotlin.test.Test
import kotlin.test.assertTrue

@AndroidIgnore
class ProverTests {
    @Test
    fun test_Prover_createLinkSecret() {
        val prover = Prover()
        val linkSecret = prover.createLinkSecret()
        println(linkSecret.getBigNumber())
        println(linkSecret.getValue())
        assertTrue(linkSecret.getBigNumber().length > 0)
    }

    @Test
    fun test_Prover_createCredentialRequest() {
        val issuer = Issuer()
        val attributeNames = listOf("name", "age")
        val schema = Schema("Moussa", "1.0", attributeNames, "sample:uri")
        val cred = issuer.createCredentialDefinition(
            "did:web:xyz/resource/schema",
            schema,
            "did:web:xyz",
            "default-tag",
            SignatureType.CL,
            CredentialDefinitionConfig(true)
        )
        val credentialOffer = issuer.createCredentialOffer(
            "did:web:xyz/resource/schema",
            "did:web:xyz/resource/cred-def",
            cred.credentialKeyCorrectnessProof
        )

        val prover = Prover()
        val linkSecret = prover.createLinkSecret()
        val credentialRequest = prover.createCredentialRequest(
            "entropy",
            null,
            cred.credentialDefinition,
            linkSecret,
            "my-secret-id",
            credentialOffer
        )
        println(credentialRequest)
        assertTrue(true)
    }

    @Test
    fun test_presentation_request_parsing() {
        val presentationRequestJson = """
            {
            "requested_attributes":{
            "attribute_1":{
            "name":"name",
            "restrictions":[

            ]
            }
            },
            "requested_predicates":{

            },
            "name":"presentation_request_1",
            "nonce":"1177620373658433495312997",
            "version":"0.1"
        }
        """
        val presentationRequest = PresentationRequest(presentationRequestJson)
        presentationRequest.getRequestedAttributes().forEach {
            println(it.key)
            println(it.value.toString())
        }
        println(presentationRequest.getRequestedPredicates())
    }
}
